import React, { useState } from "react";
import { Modal, Input, Form, Button, message } from "antd";
import { FiX } from "react-icons/fi";
import axios from "axios";

const DNSRename = ({
  visible,
  handleCancel,
  handleOk,
  doid,
  doname,
  fetchData,
}) => {
  //  ----------state -----------
  const [, setLoading] = useState(false);

  //   // ------- token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  //   // ------- apply button ---------

  const handleApply = async (data) => {
    const inputData = {
      new_domain_name: data.domain_name,
      foreign_key: { foreign_key: doid },
    };
    await axios
      .put(`${baseUrl}/settings/dns/domain_name/update`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if (res.data.operation_status === "Success") {
          message.success("Successful!");
          fetchData();
          handleOk();
          setLoading(false);
        } else {
          setTimeout(() => {
            message.error("Operation Failed! ");
          }, 1000);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        console.log(err);
        message.error("Operation Failed! ");
      });
  };

  return (
    <React.Fragment>
      <Modal
        title="Rename domain name"
        visible={visible}
        onCancel={handleCancel}
        onOk={handleOk}
        closeIcon={<FiX className="close-icon" />}
        footer={null}
      >
        <Form onFinish={handleApply} initialValues={{ domain_name: doname }}>
          <Form.Item name="domain_name">
            <Input placeholder="Text here ..." size="large" />
          </Form.Item>
          <Form.Item>
            <Button
              type="primary"
              htmlType="submit"
              size="large"
              className="button-apply2"
            >
              Submit
            </Button>
          </Form.Item>
        </Form>
      </Modal>
    </React.Fragment>
  );
};

export default DNSRename;
