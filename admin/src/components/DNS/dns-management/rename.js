import React, { useState } from "react";
import { Modal, Input, Form, Button, message } from "antd";
import { useParams } from "react-router-dom";
import { FiX } from "react-icons/fi";
import axios from "axios";
import Cookies from "js-cookie";

const DNSRename = ({ visible, handleCancel, handleOk,  zone }) => {
  //  ----------state -----------
  const [, setLoading] = useState(false);
  let { slug } = useParams();

  //   // ------- token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  // const getToken = localStorage.getItem("token");
  const getToken = Cookies.get("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  //   // ------- apply button ---------

  const handleApply = async (data) => {
    const new_domain_name = data.new_domain_name;
    const url = `${baseUrl}/settings/dns/domain_name/rename/${zone}`;
    const nextUrl = `/dns-management/${zone}/${new_domain_name}`;

    await axios
      .put(
        `${baseUrl}/settings/dns/domain_name/rename/${zone}/${slug}`,
        new_domain_name,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )

      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          window.history.pushState({ url }, null, nextUrl);
          window.location.reload();
          handleOk();
          setLoading(false);
        } else {
          setTimeout(() => {
            message.error("Operation Failed! ");
          }, 1000);
        }
      })

      .catch((err) => {
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
        <Form onFinish={handleApply} initialValues={{ new_domain_name: slug }}>
          <Form.Item name="new_domain_name">
            <Input placeholder="Text here ..." size="large" />
          </Form.Item>
          <Form.Item>
            <Button
              type="primary"
              htmlType="submit"
              size="large"
              className="button-apply2"
            >
              APPLY
            </Button>
          </Form.Item>
        </Form>
      </Modal>
    </React.Fragment>
  );
};

export default DNSRename;
