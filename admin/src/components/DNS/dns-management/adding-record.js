import React, { useState } from "react";
import { Modal, Button, Form, Input, Select, message } from "antd";
import { useParams } from "react-router-dom";

import { FiX } from "react-icons/fi";
import axios from "axios";

const { Option } = Select;

const AddRecord = ({
  handleCancel,
  handleOk,
  records,
  fetchData,
  zone,
  domainStatus,
}) => {
  //  -------------state -------------
  const [, setLoading] = useState(false);
  let { slug } = useParams();

  //   // ------- token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  //   // ------- apply button ---------

  const handleApply = async (data) => {
    const inputData = {
      domain_name: slug,
      status: domainStatus,
      zone_record: [
        {
          subdomain_name: data.subdomain_name,
          dns_type: data.dns_type,
          address: data.address,
        },
      ],
    };

    await axios
      .post(`${baseUrl}/settings/dns/new/${zone}`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          fetchData();
          handleOk();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        console.log(err);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error("Operation Failed! ");
      });
  };

  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  return (
    <React.Fragment>
      <Modal
        title={null}
        footer={null}
        closeIcon={<FiX className="close-icon" />}
        visible={records}
        onCancel={handleCancel}
        onOk={handleOk}
      >
        <div className="container-adding-records">
          <Form {...layout} size="large" onFinish={handleApply}>
            <Form.Item
              label="Sub domain name"
              name="subdomain_name"
              rules={[
                { required: true, message: "Please input sub domain name!" },
              ]}
            >
              <Input placeholder="text here ..." size="large" />
            </Form.Item>
            <Form.Item
              label="Adress"
              name="address"
              rules={[{ required: true, message: "Please input the address!" }]}
            >
              <Input placeholder="text here ..." size="large" />
            </Form.Item>
            <Form.Item
              label="Type"
              name="dns_type"
              rules={[
                {
                  required: true,
                  message: "Please select your type!",
                },
              ]}
            >
              <Select
                size="large"
                className="select-option3"
                placeholder="Select here ..."
              >
                <Option value="A">A</Option>
                <Option value="AAAA">AAAA</Option>
                <Option value="MX 10">MX 10</Option>
                <Option value="CNAME">CNAME</Option>
                <Option value="PTR">PTR</Option>
                <Option value="CERT">CERT</Option>
                <Option value="SRV">SRV</Option>
                <Option value="TXT">TXT</Option>
                <Option value="SOA">SOA</Option>
              </Select>
            </Form.Item>
            <Form.Item>
              <Button
                className="adding-record"
                size="large"
                htmlType="submit"
                type="primary"
              >
                APPLY
              </Button>
            </Form.Item>
          </Form>
        </div>
      </Modal>
    </React.Fragment>
  );
};

export default AddRecord;
